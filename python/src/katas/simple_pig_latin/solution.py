def pig_it(text: str) -> str:
    words = text.split()
    pig_latin_words = []

    for word in words:
        if word.isalpha():
            pig_latin_words.append(word[1:] + word[0] + 'ay')
        else:
            pig_latin_words.append(word)

    return ' '.join(pig_latin_words)
