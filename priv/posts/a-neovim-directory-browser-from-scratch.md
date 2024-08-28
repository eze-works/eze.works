[
    title: "A simple Neovim directory browser from scratch",
    date: ~D(2024-01-14),
    labels: ["neovim"]
]
+++
As part of my quest to replace my neovim config with 100% homemade parts, I ended up replacing netrw with my own version of a directory browser. I have never needed the remote browsing capabilities of netrw, so I figured I could replace it with something much simpler.


[Here is a video](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/assets/file-navigation.mp4).


## Disabling Netrw

Since you are replacing netrw, you should disable it. Thankfully, netrw makes it easy to do so (See `:help netrw-noload`). The following code sets 2 variables the netrw code inspects at the very start. When they are set, netrw assumes it is already loaded and does nothing.

```
vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1
```

Now when you open Neovim with a directory as target (i.e. `nvim <directory>`), you'll get an empty buffer whose name is that of the directory. The goal is to fill this buffer with the contents of the directory, one per line.

## Hijacking empty directory buffers

My approach to doing this is to hook into the `BufEnter` event. Whenever any buffer is entered, I check to see if its name is a valid directory. If it is, I replace the buffer with my custom directory browsing buffer:

```lua
vim.api.nvim_create_autocmd("BufEnter", {
  nested = true,
  callback = function(ev)
    if ev.file == "" then
      return
    end

    local path = ev.file
    local buf = ev.buf

    if vim.fn.isdirectory(path) ~= 1 then
      return
    end

    if vim.bo[buf].filetype == "cabinet" then
      return
    end

    vim.api.nvim_buf_set_name(buf, "")
    M.open(path)
    vim.api.nvim_buf_delete(buf, { force = true })
  end
})
```

A few key points:
- Bail out early if the name of the buffer opened is empty or does not refer to a directory. We do not care about these cases
- I named my custom directory browser "Cabinet", so I also bail out if the buffer being opened is a cabinet buffer. Remember, this hook is just about transforming an empty directory buffer into one that actually lists the contents.
- To actually do the replacement, in my implementation I decided to _delete_ the currently empty directory buffer and create my own with the same name. (Though I imagine you _could_  just reuse the empty buffer and write the directory contents to it). Because buffer names are unique, I have to rename the empty directory buffer first before I can create my own custom buffer with the same name.


## Creating our custom directory buffer

The core of this little project is in the `M.open(path)` function call. It creates a new custom directory buffer for the given path and displays it in the current window. However, if a custom directory buffer already exists for that path, it reuses it and displays that.

Steps:
1. If there is an existing buffer with the same filetype as your custom buffer (see below) and the same name, display that buffer and return early ([code](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L73)).
1. Create a new buffer using `vim.api.nvim_create_buf()`
1. Set some buffer options on it ([code](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L85)). I think the most important once are
   1. `filetype = <yourchoice>` : Giving your directory buffer a custom filetype makes it easy to identify.
   1. `buflisted = false`: You'll end up with a separate buffer for each directory you visit. Setting `buflisted` to false will make it such that the directory buffers are not shown with `:ls`. This is optional. 
   1. `buftype = nofile`: Your directory buffer does not represent a file on disk that can be written to. This tells vim that the buffer can't be written to and is never considered "modified".
   1. `modifiable = false`: This prevents insert mode in the buffer.
1. Register a buffer-local autocommand that prints the directory contents to the buffer whenever it is entered ([code](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L99)).
1. Register a mapping for the `<Enter>` key that executes `:edit` on whatever the user entered ([code](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L109)).
   1. For files, this would edit the file as normal. For directories, this will tie into one of our earlier `BufEnter` autocommands and display a custom directory buffer.

And that's all the setup needed. Now for actually printing the contents of a directory to a buffer...

## Printing the contents of a directory to a buffer

This is the most straightforward part. I [used the `vim.fs.dir()`](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L14) function. It returns an iterator over the contents of the directory. Each iteration yields two values. The second value tells you if it is a file or directory. This makes it easy to show directories first for example.

Gather the iteration results into a list. And write the list to  the buffer using `vim.api.nvim_buf_set_lines()` ([code](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L44)).

If you want to give directories a different highlight, sorting the directories first actually makes things easier. Assuming there are N directories, [you blindly set the highlight for the first N lines](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/db089219675cccb2a9016daa195368d2fbc80830/lua/cabinet.lua#L59) to whatever you want.

## Custom user command for navigating to the current directory

Most directory browsing plugins allow you to hit "-" to go up a level: If you are in a file buffer, it displays the directory that file is in, and if you are in a directory buffer, it displays the parent directory.

The first step to getting that working is to create a custom command. This is the entire code for the custom command:

```lua
vim.api.nvim_create_user_command("Cabinet", function()
  local path = vim.api.nvim_buf_get_name(0)
  local parent = nil
  if vim.fn.filereadable(path) == 1 or vim.fn.isdirectory(path) then
    parent = vim.fs.dirname(path)
  else
    parent = vim.fn.getcwd()
  end
  M.open(parent)
end, {})
```

Nothing too surprising here. The `M.open(parent)` function call is the same `open()` function we saw in the previous section. This even works from buffers that represent neither file nor directory (e.g. the quickfix list).

Map this the "-" to ":Cabinet<CR>" and you are done!

## Potential Improvements

What I have outlined so far already gets you all the directory navigation you need. Because the directory buffers are named after directory, you can create new files by doing:

```
:edit %/new_file_name
```

That's nice, but you could do so much more. I want to take inspiration from emacs dired ([cool video](https://www.youtube.com/watch?v=8l4YVttibiI)) and see if i can implement things such as:
- Printing more file attributes to the buffer (e.g. size) 
- Editing the `mode` bits of a file (this would be helpful to quickly make a script executable)
- Renaming a file by just editing the part of the buffer that corresponds to the file name.
  - I got a slightly hacky proof of concept of this going using the `CursorMovedI` autocommand to limit where the cursor can be and the `virtualedit` option to snap the cursor to a location within the line. Still some kinks to work out though

Regardless of further progress, it always nice to scratch the "Not Invented Here" itch once more as I gradually make a vertically integrated Neovim config ... who knows what's next? ... huh .. maybe I should take a stab at my own plugin manager next ... ha ha ... the irony.
