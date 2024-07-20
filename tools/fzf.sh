#!/bin/bash

# Select a file using fzf
selected_file=$(rg --files / 2>/dev/null | fzf)

# Extract the file extension
extension="${selected_file##*.}"

# Define applications associated with file extensions
firefox=("pdf")
eog=("jpg" "png" "gif" "bmp" "tiff")
vlc=("mp3" "wav" "flac" "aac" "mp4" "avi" "mkv" "mov")
xdg_open=("pdf" "jpg" "png" "gif" "bmp" "tiff" "mp3" "wav" "flac" "aac" "mp4" "avi" "mkv" "mov" "zip" "tar.gz" "rar" "7z")
sqlite3=("db" "sqlite")
libreoffice=("docx" "xlsx" "pptx")
hex_editor=("bin")

# Function to check if extension is in array
contains() {
    local ext=$1
    shift
    local arr=("$@")
    for item in "${arr[@]}"; do
        if [[ "$item" == "$ext" ]]; then
            return 0
        fi
    done
    return 1
}

# Open the file based on its extension
if contains "$extension" "${firefox[@]}"; then
    firefox "$selected_file"
elif contains "$extension" "${eog[@]}"; then
    eog "$selected_file"
elif contains "$extension" "${vlc[@]}"; then
    vlc "$selected_file"
elif contains "$extension" "${xdg_open[@]}"; then
    xdg-open "$selected_file"
elif contains "$extension" "${sqlite3[@]}"; then
    sqlite3 "$selected_file"
elif contains "$extension" "${libreoffice[@]}"; then
    libreoffice "$selected_file"
elif contains "$extension" "${hex_editor[@]}"; then
    hex_editor "$selected_file"
else
    # If not handled by any application, open with Vim or print directory
    if [[ "$extension" == "sh" || "$extension" == "txt" || "$extension" == "md" ]]; then
        vim "$selected_file"
    else
        echo "File cannot be opened by Vim. Directory of the file:"
        dirname "$selected_file"
    fi
fi

