# If you come from bash you might have to change your $PATH.
# export PATH=$$HOME/bin:/usr/local/bin:$PATH

export ZSH="$HOME/.oh-my-zsh"

#ZSH_THEME="gnzh"
ZSH_THEME="half-life"

plugins=(
    git
    archlinux
    zsh-autosuggestions
    zsh-syntax-highlighting
)

source $ZSH/oh-my-zsh.sh

# Check archlinux plugin commands here
# https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/archlinux

# Display Pokemon-colorscripts
# Project page: https://gitlab.com/phoneybadger/pokemon-colorscripts#on-other-distros-and-macos
#pokemon-colorscripts --no-title -s -r

# Set-up icons for files/folders in terminal
alias vmrs="$HOME/scripts/Personal/bash_scripts/vmrss.sh"
alias btop="btop --utf-force"
alias vi="nvim"
alias nv="nvim"
alias cat="bat"
alias scr="cd $HOME/scripts/"
alias z="nvim /$HOME/.zshrc"
alias zs="source /$HOME/.zshrc"
alias f="$HOME/scripts/Personal/tools/fzf/target/debug/fzf"
alias blc="$HOME/scripts/Personal/bash_scripts/blt.sh c"
alias bld="$HOME/scripts/Personal/bash_scripts/blt.sh d"
alias tht="/opt/temp-throttle/temp-throttle"
alias fwx="$HOME/scripts/Personal/bash_scripts/file_search.sh fx"
alias fwn="$HOME/scripts/Personal/bash_scripts/file_search.sh fn"
alias etd="$HOME/scripts/Personal/bash_scripts/end_day.sh"
alias std="$HOME/scripts/tmp/bash_scripts/start_day.sh"
alias kubectl="k3s kubectl"
 
fastfetch

# Set-up FZF key bindings (CTRL R for fuzzy history finder)
source <(fzf --zsh)

HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
setopt appendhistory
export PATH=$HOME/.local/bin:$PATH
export PATH=/opt/zen/:$PATH
