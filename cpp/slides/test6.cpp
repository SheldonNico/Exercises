struct SomeClass {};
class MyClass {
  public:
    MyClass() { data_ = new SomeClass; }
    ~MyClass() {
      delete data_;
      data_ = nullptr;
    }

  private:
    SomeClass* data_;
};

int main() {
  MyClass a;
  MyClass b(a);
  return 0;
}
