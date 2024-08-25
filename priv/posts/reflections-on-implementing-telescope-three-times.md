+++
[
    title: "Reflections on implementing a homemade Telescope/fuzzy finder (3 times)",
    date: ~D(2024-01-07),
    labels: ["editors/neovim"],
]
+++

Whoever said "third time's the charm" was a prophet, because I have done it again: [implementing a thing three times](/post/reflections-on-writing-3-parser-combinator-libraries") before being satisfied with the final result. This time it's a homegrown version of the popular neovim plugin [Telescope](https://github.com/nvim-telescope/telescope.nvim).

_Attempt 1_: A floating window layout. I had not yet figured out how to do the preview.

![first-attempt](/assets/images/telescope-first-attempt.png)

_Attempt 2_: A semi-floating layout (the search window is floating). I found an OK way to do previews.

![second-attempt](/assets/images/telescope-second-attempt.png)

_Attempt 3 (aka the charm)_: A non-floating (sinking?) layout. Finally figured out how to have proper previews.

![third-attempt](/assets/images/telescope-third-attempt.png)

This was part of a project to [re-write my neovim config without using any third-party plugins](https://gitlab.com/wake-sleeper/plugin-free-neovim). [Here is my old config](https://gitlab.com/wake-sleeper/dotfiles/-/blob/383833fca0011d6f9f85248a5ef66a1f104d4ae5/nvim/.config/nvim/init.lua) so you get an idea of what i was trying to replicate.

__Event-driven UIs are much easier to reason about__

My first and third attempts were event-driven. I am not sure how common this is, but I used the `User` event to create custom events I can dispatch and listen for. You can think of each window as a component that can send and listen for events. The basic flow is this:

- The search window accepts user input and key presses. However, it can't do anything with them so [it dispatches those actions as events](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/8663c85081fda56592ba2ebcf95e63bb92d3902d/lua/uscope.lua#L462).

- The results window listens for navigation key press events, and moves the cursor in response. It then dispatches an [event indicating that the cursor is on a new entry](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/8663c85081fda56592ba2ebcf95e63bb92d3902d/lua/uscope.lua#L343).

- The preview window's only job is to [watch for these `new_selection` events](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/8663c85081fda56592ba2ebcf95e63bb92d3902d/lua/uscope.lua#L273) and preview the thing under the cursor.

The result is a system that is much easier to reason about than my second attempt where things were more sequential. It is also much easier to refactor and cleanup. All things in moderation though! I had an attempt 1.5 that never got anywhere because I became trigger happy with events and went too far[^1]


__`vim.api.nvim_buf_call()` exists__

There are a handful of occaisions where I needed to execute an `Ex` command or a function in the context of another window that contains a certain buffer. My approach was to (1) save the current window then (2) switch to the target window to do whatever I needed to do and finally (3)  switch back. This usually worked, but there are some dark corners that where it didn't. I won't bore you with the details, but I ran [into this](https://github.com/neovim/neovim/issues/21437). Curious, I started looking through the Telescope code to figure out how they do it and I discovered [this gem](https://github.com/nvim-telescope/telescope.nvim/blob/87e92ea31b2b61d45ad044cf7b2d9b66dad2a618/lua/telescope/previewers/buffer_previewer.lua#L293C6-L293C6). This code uses `vim.api.nvim_buf_call()` to temporarily switch to the Help tag preview window, scroll to the currently selected tag, and highlight it. Works flawlessly. I promptly [lifted those ~4 lines and put them in my config](https://gitlab.com/wake-sleeper/plugin-free-neovim/-/blob/8663c85081fda56592ba2ebcf95e63bb92d3902d/lua/uscope.lua#L183). Suuuch a nice pattern. Thank you to the neovim dev(s) that implemented it!

__Computers are faaast__

There is a lot of talk among plugin developers (or at least that's how I perceive it) about the importance of doing things in an async manner. Before building this, I too assumed I would have to use `jobstart()`. I mean, I am making calls to an external executable and updating buffers on every user key press! Surely the user will notice some sluggishness if the executable calls are happening synchronously. Well... turns out no. I call `vim.fn.system(...)`, repaint the result screen and repaint the preview screen on every key press. The preview screen also reads the selected file on every key press too. I don't event try to cache anything. Its practically instantaneous[^2]. My intuition of how fast computers can get things done is way off.

[^1]: I had just discovered lua metatables, and so i combined it with my newly-discovered custom event knowledge to create an unholy match: Setting properties on a global `State` table would trigger events. So for example setting `State.opened` would trigger an event that would open the filter UI. I think there are many reasons this is a bad practice. It bit me for two main reasons though: (1) It became difficult to remember which properties trigger events and (2) Setting the state optimistically before actually trying to _do the thing_ gets you into messy ground when _the thing_ fails to be done. Now your state is out of whack.

[^2]: Naturally, it would depend on the codebase you are navigating. I tried it on my work's codebase and on Neovim's codebase. No noticeable difference. It perhaps would start to matter if you are working on something the size of the Linux kernel maybe? But ... are you? No? Ok.
