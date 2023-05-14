from flask import Flask
from youtube_transcript_api import YouTubeTranscriptApi

app = Flask(__name__)

def get_string_format(json_list: list) -> str:
    """
    To formats the transcript JSON into a string format, follow as given.

    Args:
    ----
        json_list (list): A list of transcript segments in JSON format.

    Returns:
    -------
        string (str): A formatted string containing the transcript text.
    """
    string = ""
    for segment in json_list:
        string += segment["text"]+"."
        string += " "
    return string

@app.route("/get_transcript/<video>",methods=["GET"])
def index(video):
    json_list = YouTubeTranscriptApi.get_transcript(video, languages=["en", "en-US"])
    string_format = get_string_format(json_list)
    print("return transcript")
    return {"transcript":string_format.lower()}

if __name__ == "__main__":
    app.run(debug=True,host="localhost",port=8000)
