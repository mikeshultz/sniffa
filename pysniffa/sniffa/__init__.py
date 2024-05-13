import rsniffa

def version() -> str:
    """ Return version of the library"""
    return rsniffa.version()

def capture_text():
    print('capture starting...')
    for line in rsniffa.TextSniffer():
        print(line)
