# If you come from bash you might have to change your $PATH.
# export PATH=$HOME/bin:/usr/local/bin:$PATH

export ZSH="$HOME/.oh-my-zsh"

#ZSH_THEME="gnzh"
ZSH_THEME="robbyrussell"

plugins=(
    git
    archlinux
    zsh-autosuggestions
    zsh-syntax-highlighting
)

source $ZSH/oh-my-zsh.sh
export PATH="$HOME/.cargo/bin:$PATH"
# Check archlinux plugin commands here
# https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/archlinux

# Display Pokemon-colorscripts
# Project page: https://gitlab.com/phoneybadger/pokemon-colorscripts#on-other-distros-and-macos
#pokemon-colorscripts --no-title -s -r

# Set-up icons for files/folders in terminal
alias ls='eza -a --icons'
alias ll='eza -al --icons'
alias lt='eza -a --tree --level=1 --icons'
alias vmrs="$HOME/scripts/tools/vmrss.sh"
alias btop="btop --utf-force"
alias vi="nvim"
alias nv="nvim"
alias cat="bat"
alias scr="cd $HOME/scripts/"
alias f="$HOME/scripts/tools/fzf/target/debug/fzf"
alias z="nvim ~/.zshrc"
alias zs="source ~/.zshrc"
alias cfg="cd $HOME/.config/"
alias clip="xclip -selection clipboard -o"
#quotes
$HOME/scripts/mh-qt/run_mhqt.sh | cowsay -f tux


#fzf
#fastfetch
# Set-up FZF key bindings (CTRL R for fuzzy history finder)
source <(fzf --zsh)

HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
setopt appendhistory
export PATH=$HOME/.local/bin:$PATH
export PATH=$PATH:/home/vamsi/.local/share/gem/ruby/3.0.0/bin

alias luamake="/opt/lua-language-server/3rd/luamake/luamake"
export PATH="/opt/lua-language-server/bin/:$PATH"
