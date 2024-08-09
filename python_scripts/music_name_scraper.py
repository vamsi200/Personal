import youtube_dl

# Replace this URL with your playlist URL
playlist_url = "https://music.youtube.com/playlist?list=PLOgAesuggbpb2V6seZLJcD7Mw73hoiSAI&si=iC6b-Vs4HvHBB3n9"
output_file = "output.txt"

def get_playlist_titles(url):
    ydl_opts = {
        'quiet': True,
        'extract_flat': True,
        'force_generic_extractor': True,
        'playlist_items': '1-100',
        'playlist': True,
    }

    with youtube_dl.YoutubeDL(ydl_opts) as ydl:
        info_dict = ydl.extract_info(url, download=False)
        entries = info_dict.get('entries', [])

        with open(output_file, 'w') as f:
            for entry in entries:
                f.write(entry.get('title') + '\n')            



if __name__ == "__main__":
    get_playlist_titles(playlist_url)

