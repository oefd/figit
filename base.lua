local pacman = require("state").pacman
local _ = require("output").salt_like

_(pacman.installed {
    id = "install fonts",
    pkgs = { "ttf-liberation" },
})
