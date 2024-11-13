vim.opt.rtp:prepend("~/.local/share/nvim/lazy/lazy.nvim")
vim.wo.number = true
vim.wo.relativenumber = true
vim.opt.laststatus = 0
require("lazy").setup({
    { "blazkowolf/gruber-darker.nvim" },
    { "williamboman/mason.nvim", build = ":MasonUpdate" },
    "williamboman/mason-lspconfig.nvim",
    "neovim/nvim-lspconfig",
    "hrsh7th/nvim-cmp",
    "hrsh7th/cmp-nvim-lsp",
    "L3MON4D3/LuaSnip",
    { 'nvim-telescope/telescope.nvim', dependencies = { 'nvim-lua/plenary.nvim' } },
    -- Add rust-tools for enhanced Rust development
    { 'simrat39/rust-tools.nvim' },
})

vim.cmd([[colorscheme gruber-darker]])
require("mason").setup()

local mason_lspconfig = require("mason-lspconfig")
mason_lspconfig.setup({
    ensure_installed = { "clangd", "rust_analyzer", "pyright", "bashls" },
    automatic_installation = true,
})

local lspconfig = require('lspconfig')
local rt = require('rust-tools')

-- Configure rust-tools
rt.setup({
    server = {
        settings = {
            ["rust-analyzer"] = {
                cargo = { allFeatures = true },
                checkOnSave = {
                    command = "clippy",
                },
                -- Enable automatic formatting on save
                procMacro = {
                    enable = true
                },
            },
        },
        on_attach = function(client, bufnr)
            -- Enable formatting on save for Rust files
            vim.api.nvim_create_autocmd("BufWritePre", {
                pattern = "*.rs",
                callback = function()
                    vim.lsp.buf.format({ timeout_ms = 2000 })
                end,
            })
        end,
    },
})

-- Configure other LSP servers
local servers = {
    pyright = {},
    bashls = {},
    clangd = {},
}

for lsp, config in pairs(servers) do
    lspconfig[lsp].setup(config)
end

-- Rest of your existing configuration
local cmp = require('cmp')
local luasnip = require('luasnip')
cmp.setup({
    snippet = {
        expand = function(args)
            require('luasnip').lsp_expand(args.body)
        end,
    },
    mapping = {
        ['<Tab>'] = cmp.mapping(function(fallback)
            if cmp.visible() then
                cmp.select_next_item()
            elseif luasnip.expand_or_jumpable() then
                luasnip.expand_or_jump()
            else
                fallback()
            end
        end, { 'i', 's' }),
        
        ['<S-Tab>'] = cmp.mapping(function(fallback)
            if cmp.visible() then
                cmp.select_prev_item()
            elseif luasnip.jumpable(-1) then
                luasnip.jump(-1)
            else
                fallback()
            end
        end, { 'i', 's' }),
        
        ['<Down>'] = cmp.mapping.select_next_item(),
        ['<Up>'] = cmp.mapping.select_prev_item(),
        
        ['<C-Space>'] = cmp.mapping.complete(),
        ['<CR>'] = cmp.mapping.confirm { select = true },
        ['<Esc>'] = cmp.mapping.close(),
    },
    sources = cmp.config.sources({
        { name = 'nvim_lsp' },
        { name = 'luasnip' },
    })
})

vim.g.mapleader = " "
vim.api.nvim_set_keymap('n', '<leader>n', ':Explore<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('n', '<leader>v', ':vsplit | terminal<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('n', '<leader>h', ':split | terminal<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('i', '<C-e>', '<Esc>A', { noremap = true, silent = true })
vim.api.nvim_set_keymap('i', '<C-a>', '<C-o>^', { noremap = true, silent = true })
vim.api.nvim_set_keymap('i', '<C-H>', '<C-w>', { noremap = true, silent = true })
vim.o.completeopt = 'menuone,noselect'
vim.opt.clipboard = 'unnamedplus'

local telescope = require('telescope')
telescope.setup()
vim.api.nvim_set_keymap('n', '<leader>ff', ':Telescope find_files<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('n', '<leader>fg', ':Telescope live_grep<CR>', { noremap = true, silent = true })

-- Keep your existing format on save for C/C++ files
vim.api.nvim_create_autocmd("BufWritePre", {
    pattern = {"*.c", "*.cpp", "*.h"},
    callback = function()
        vim.lsp.buf.format()
    end,
})

vim.api.nvim_set_keymap('n', '<leader>cf', ':lua vim.lsp.buf.format()<CR>', { noremap = true, silent = true })
