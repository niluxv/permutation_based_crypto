import kravatte

def kravatte_generate(key: bytes, msgs: list[bytes], out_size: int) -> list[bytes]:
    kra = kravatte.Kravatte(key)
    for msg in msgs:
        kra.collect_message(msg)
    kra.generate_digest(out_size)
    out = kra.digest
    return out

def fmt_out(out: bytes) -> str:
    fmt = "["
    for b in out:
        fmt += hex(b) + ", "
    fmt += "]"
    return fmt

def testcase(n: int, msgs: list[bytes], out_size: int = 32):
    key = b"kravatte test key"
    out = kravatte_generate(key, msgs, out_size)
    print(f"Testcase {n}:")
    print("-----------")
    print(fmt_out(out))
    print("-----------")
    print()

def case1():
    msgs = [b"hello world"]
    testcase(1, msgs)

def case2():
    msgs = [b"hello", b"world"]
    testcase(2, msgs)

def case3():
    msgs = [b"hello world"]
    testcase(3, msgs, out_size = 4*32)

case1()
case2()
case3()
