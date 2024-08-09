import os
import yt_dlp

def load_track_names(filename):
    with open(filename, 'r') as f:
        return [line.strip() for line in f]

def search_youtube(track_name):
    ydl_opts = {
        'quiet': True,
        'noplaylist': True,
        'format': 'bestaudio/best',
        'extract_flat': True,
        'force_generic_extractor': True,
        'default_search': 'ytsearch',
        'search_query': track_name
    }
    
    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        result = ydl.extract_info(f"ytsearch:{track_name}", download=False)
        if 'entries' in result:
            return result['entries'][0]['url']
    return None

def download_audio(youtube_url, output_dir):
    ydl_opts = {
        'outtmpl': os.path.join(output_dir, '%(title)s.%(ext)s'),
        'format': 'bestaudio/best',
        'postprocessors': [{
            'key': 'FFmpegExtractAudio',
            'preferredcodec': 'mp3',
            'preferredquality': '192',
        }],
        'quiet': True
    }
    
    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        ydl.download([youtube_url])

def main():
    track_file = 'output.txt'  # File containing the list of song names
    output_dir = 'Music'  # Directory to save downloaded music

    track_names = load_track_names(track_file)

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
        print(f"Created directory: {output_dir}")

    for track_name in track_names:
        youtube_url = search_youtube(track_name)
        if youtube_url:
            download_audio(youtube_url, output_dir)
        else:
            print(f"No results found for {track_name}")

if __name__ == "__main__":
    main()

