import os
import sys
from counter import Counter

def example(lib):
    counter = Counter("target/release/libfrom.so")
    print("value after init:{}".format(counter.value()))
    counter.inc()
    counter.inc()
    counter.inc()
    print("value after some inc:{}".format(counter.value()))
    counter.clear()
    print("value after clear:{}".format(counter.value()))

if __name__ == "__main__":
    cur_dir = os.path.dirname(os.path.realpath(__file__))
    lib = os.path.join(cur_dir, "..", "..", "target/release/libfrom.so")
    sys.path.append(cur_dir)
    example(lib)
