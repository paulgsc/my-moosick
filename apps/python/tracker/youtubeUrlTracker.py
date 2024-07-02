import time
from browser_history.browsers import Chrome
from urllib.parse import urlparse, parse_qs

def is_youtube_video_url(url):
    parsed_url = urlparse(url)
    return parsed_url.netloc == "www.youtube.com" and parsed_url.path == "/watch"

def get_video_id(url):
    parsed_url = urlparse(url)
    return parse_qs(parsed_url.query).get('v', [None])[0]

def main():
    chrome = Chrome()
    last_url = None
    
    print("Monitoring Chrome history for YouTube URLs...")
    
    while True:
        outputs = chrome.fetch_history()
        if outputs.histories:
            latest_url = outputs.histories[-1][1]
            
            if latest_url != last_url and is_youtube_video_url(latest_url):
                video_id = get_video_id(latest_url)
                if video_id:
                    print(f"New YouTube video detected: {latest_url}")
                    with open("youtube_urls.txt", "a") as f:
                        f.write(f"{latest_url}\n")
                    last_url = latest_url
        
        time.sleep(5)  # Check every 5 seconds

if __name__ == "__main__":
    main()