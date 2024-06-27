def generate_hashtag(s: str) -> str|bool:
    hashtag = '#' + ''.join(word.capitalize() for word in s.split())
    if not s or len(hashtag) > 140:
        return False
    return hashtag
