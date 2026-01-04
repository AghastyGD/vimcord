local M = {}

M.config = {
    enabled = true,
    debounce_ms = 1000,
}

local last_time = 0

local function should_run()
    local now = vim.loop.now()
    if now - last_time < M.config.debounce_ms then
        return false
    end
    last_time = now
    return true
end

-- Send update to vimcord CLI
local function update_presence()
    if not M.config.enabled then
        return
    end

    if not should_run then
        return
    end

    local file = vim.fn.expand("%:t")
    if file == "" then
        return
    end

    local cwd = vim.fn.fnamemodify(vim.loop.cwd(), ":t")

    local cmd = {
        "vimcord",
        "update",
        "--file", file,
        "--workspace", cwd,
        "--details", "In Vim",
        "--state", "Editing",
    }

    vim.fn.jobstart(cmd, {detach = true})
end

function M.setup(opts)
    M.config = vim.tbl_extend("force", M.config, opts or {})

    vim.api.nvim_create_autocmd(
        { "BufEnter", "BufWritePost" },
        {
            callback = update_presence,
            desc = "Update Discord Rich Presence via vimcord",
        }
    )
end

return M