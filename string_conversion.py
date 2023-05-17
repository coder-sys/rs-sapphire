import os
import nltk
from nltk.sentiment import SentimentIntensityAnalyzer
from dotenv import load_dotenv
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError
from isodate import parse_duration
nltk.download("punkt")
nltk.download('vader_lexicon')
load_dotenv()

def join_most_sophisticated_sentences(text, num_sentences):
    scores = []
    """
    Follow as given to join the most sophisticated sentences of a text (string) together.

    Args:
    ----
        text (str): The input text.
        num_sentences (int): The number of sophisticated sentences to join.

    Returns:
    -------
        str: The joined sophisticated sentences of the input text.
    """
    if len(text) == 0:
        return ""

    # Tokenize the input text into sentences
    sentences = nltk.sent_tokenize(text)

    # Initialize the Sentiment Intensity Analyzer
    sid = SentimentIntensityAnalyzer()

    # Calculate the sentiment scores for each sentence
    sentence_scores = {}
    for sentence in sentences:
        sentiment_scores = sid.polarity_scores(sentence)
        sentence_scores[sentence] = sentiment_scores
    # Sort the sentences based on their compound sentiment score in descending order
    sorted_sentences = sorted(
        sentence_scores.items(), key=lambda x: x[1]["compound"], reverse=True
    )
    # Extract the most sophisticated sentences
    sophisticated_sentences = [
        sentence for (sentence, _) in sorted_sentences[:num_sentences]
    ]

    # Join the most sophisticated sentences back into a single string
    sophisticated_text = " ".join(sophisticated_sentences)

    return sophisticated_text
def modify_transcript(
    limit1: str, limit2: str, video_id: str, string_format: str
) -> str:
    """
    Follow these steps to modify the transcript string based on the duration of the video.

    Args:
    ----
        limit1 (str): The lower limit of the video duration range to modify the transcript for.
        limit2 (str): The upper limit of the video duration range to modify the transcript for.
        video_id (str): The ID of the YouTube video to retrieve the transcript for.
        string_format (str): The original transcript string to modify.

    Returns:
    -------
        transcript (str): The modified transcript string.
    """
    if get_youtube_video_duration(video_id, os.getenv("api_key")) >= float(
        limit1
    ) and get_youtube_video_duration(video_id, os.getenv("api_key")) <= float(limit2):
        transcript = join_most_sophisticated_sentences(string_format, 50)
        print("Using method 2")
        return transcript
    else:
        print("Using method 1")
        return string_format


def get_youtube_video_duration(video_id: str, api_key: str) -> float:
    """
    Follow as given to return the duration of a YouTube video given its video ID using the YouTube Data API.

    Args:
    ----
        video_id (str): The ID of the YouTube video.
        api_key (str): Your YouTube Data API key.

    Returns:
    -------
        float: The duration of the video in seconds.
    """

    youtube = build("youtube", "v3", developerKey=api_key)

    try:
        video_response = (
            youtube.videos().list(part="contentDetails", id=video_id).execute()
        )

        duration = parse_duration(
            video_response["items"][0]["contentDetails"]["duration"]
        ).total_seconds()
        return duration / 60
    except HttpError as e:
        print(f"An error occurred: {e}")

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

def get_youtube_video_duration(video_id: str, api_key: str) -> float:
    """
    Follow as given to return the duration of a YouTube video given its video ID using the YouTube Data API.

    Args:
    ----
        video_id (str): The ID of the YouTube video.
        api_key (str): Your YouTube Data API key.

    Returns:
    -------
        float: The duration of the video in seconds.
    """

    youtube = build("youtube", "v3", developerKey=api_key)

    try:
        video_response = (
            youtube.videos().list(part="contentDetails", id=video_id).execute()
        )

        duration = parse_duration(
            video_response["items"][0]["contentDetails"]["duration"]
        ).total_seconds()
        return duration / 60
    except HttpError as e:
        print(f"An error occurred: {e}")

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
        string += segment["text"]
        string += " "
    return string
def sent_tokenize(string):
    sent_tokens = nltk.sent_tokenize(string)
    return sent_tokens