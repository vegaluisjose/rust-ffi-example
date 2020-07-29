class Counter {
  int val;
  public:
    Counter();
    void clear();
    void inc();
    int value();
};

Counter::Counter() {
  val = 0;
}

void Counter::clear() {
  val = 0;
}

void Counter::inc() {
  val += 1;
}

int Counter::value() {
  return val;
}

#ifdef __cplusplus
extern "C" {
#endif

typedef void* Handle;

Handle alloc() {
  Counter* counter = new Counter;
  return static_cast<Handle>(counter);
}

void clear(Handle handle) {
  Counter* counter = static_cast<Counter*>(handle);
  counter->clear();
}

void inc(Handle handle) {
  Counter* counter = static_cast<Counter*>(handle);
  counter->inc();
}

int value(Handle handle) {
  Counter* counter = static_cast<Counter*>(handle);
  return counter->value();
}

void dealloc(Handle handle) {
  delete static_cast<Counter*>(handle);
}

#ifdef __cplusplus
}
#endif
