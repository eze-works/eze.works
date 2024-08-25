+++
[
    title: "Integrating TUIs into Neovim",
    date: ~D(2024-02-02),
    labels: ["editors/neovim"]
]
+++
At the end of my [previous post](/post/a-neovim-directory-browser-from-scratch), I pointed out potential improvements to be made to my lua directory browser. None of those came to fruition, but not due to lack of effort.

I was hoping to re-create something like what [oil.nvim](https://github.com/stevearc/oil.nvim) does where you can edit your directory like you would edit a regular buffer.
I wanted it to work cleanly without any possibility for messing up the buffer. In `oil.nvim` you can get the directory buffer into a unreadable state by shifting any line over (`>>`) then modifying the line ids that appear on the left. Attempting to save will now throw an error. This doesn't sit right with me because I expect user interfaces to be "tamper-proof". But no matter how hard I tried, I could not find a way to make a regular buffer behave like a tamper-proof user interface.

And then I remembered that Neovim allows you to spawn a terminal buffer. It turns out that spawning Terminal User Interfaces (TUIs) in a Neovim terminal can work extremely well. These TUIs can be arbitrarily complex and they'll work as you expect. It makes you wonder why you would use a neovim file manager implemented in lua when you can use a terminal file manager and embed it into Neovim.

Here are some tips I gathered while embedding various TUIs into my Neovim workflow:

## Refrain from binding keys in terminal mode

Terminal buffers forward all key presses except `<C-\><C-N>` to the running process. You should keep it that way! I had foolishly rebound `<Esc>` to `<C-\><C-N>` to allow me to quickly switch out of terminal buffers. But that meant that terminal processes could no longer receive the `<Esc>` key (e.g. exiting the current pane in `lazygit`). How do you easily switch out of terminal buffers then?

## Prefer TUIs that let you customize bindings 

A great example of this is the terminal file manager [`lf`](https://github.com/gokcehan/lf). You can spawn it with custom configuration that creates custom commands. I rebound the `<Esc>` key  (in my `lf` config) to a custom `switch` command that executes the following shell incantation:

```shell
nvim --server {running-vim-instance} --remote-send "<C-\\><C-N>:lua require('lf').lf_switch()<CR>"
```

Note: You would replace `{running-vim-instance}` with the address you get from `vim.v.servername`.

This uses Neovim's remote feature to forward an Ex command to a running Neovim process from a shell process. Neat! The Ex command executes a lua function called `lf_switch()`. That function does `vim.cmd("buffer #")` to switch to displaying the alternate file (See `:help alternate-file`) in the window

Here is a diagram:

![Eze's blog](/assets/images/neovim-remote-diagram.png)


## Always start terminal mode in insert mode

This tip is actually documented in `:help :terminal`. But I found you might need to do a bit more if you have a setup such that you can switch back to a terminal buffer (instead of exiting the terminal process when you are done with it). Here is a snippet from my config [^1]:

```lua
-- Terminals should always start with insert mode so that keys are immediately
-- forwarded to whatever process is running within
vim.api.nvim_create_autocmd("TermOpen", {
  callback = function(cmd_args)
    core.startinsert()

    -- Thereafter, any `BufEnter` events into this buffer should also trigger insert mode
    vim.api.nvim_create_autocmd("BufEnter", {
      buffer = cmd_args.buf,
      callback = function()
        core.startinsert()
        -- When displaying an existing terminal buffer in a different window,
        -- its contents are still drawn as if they were in the original window
        -- the buffer was created in. Redrawing fixes that
        vim.cmd("redraw")
      end
    })
  end
})
```

---


The combination of these techniques has allowed me to seamlessly use `lf` and `lazygit` without leaving Neovim. It Just Works (TM).


_`lf` integration:_
![lf-integration](/assets/images/neovim-lf-integration.png)

_`lazygit` integration:_
![lazy-git-integration](/assets/images/neovim-lazygit-integration.png)


[^1]: `core.startinsert()` is just a wrapper for `vim.schedule(function() vim.cmd("startinsert")end)`. I have generally found that `:startinsert` just doesn't seem to work sometimes unless you defer it. I have no idea why :/
