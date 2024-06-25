def generate_hashtag(s: str) -> str|bool:
    if not s or len(s) > 140:
        return False
    hashtag = '#' + ''.join(word.capitalize() for word in s.split())
    return hashtag
