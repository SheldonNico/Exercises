#include <mutex>
#include <thread>
#include <deque>
#include <queue>
#include <condition_variable>
#include <memory>
#include <future>
#include <string>

template<typename T, typename Container = std::deque<T>>
class queue {
  public:
    explicit queue(const Container&);
    explicit queue(Container&& = Container());

    template<typename Alloc> explicit queue(const Alloc&);
    template<typename Alloc> explicit queue(const Container&, const Alloc&);
    template<typename Alloc> explicit queue(Container&&, const Alloc&);
    template<typename Alloc> explicit queue(queue&&, const Alloc&);

    void swap(queue& q);
    bool empty() const;
    size_t size() const;

    T& front();
    const T& front() const;
    T& back();
    const T& back() const;

    void push(const T& x);
    void push(T&& x);

    void pop();
    template<typename... Args> void emplace(Args&&... args);
};

template<typename T>
class threadsafe_queue {
  private:
    mutable std::mutex m_mut;
    std::queue<T> m_data;
    std::condition_variable m_cond;
  public:
    threadsafe_queue() {}
    threadsafe_queue(const threadsafe_queue& other) {
      std::lock_guard<std::mutex> lock{other.m_mut};
      m_data = other.m_data;
    }
    threadsafe_queue& operator=(const threadsafe_queue&) = delete;

    void push(T new_value) {
      std::lock_guard<std::mutex> lock{m_mut};
      m_data.push(new_value);
      m_cond.notify_one();
    }
    bool try_pop(T& value) {
      std::lock_guard<std::mutex> lock{m_mut};
      if (m_data.empty()) {
        return false;
      } else {
        value = m_data.front();
        m_data.pop();
        return true;
      }
    }
    std::shared_ptr<T> try_pop() {
      std::lock_guard<std::mutex> lock{m_mut};
      if (m_data.empty()) {
        return std::make_shared<T>();
      } else {
        auto out= std::make_shared<T>(m_data.front());
        m_data.pop();
        return out;
      }
    }

    void wait_and_pop(T& value) {
      std::unique_lock<std::mutex> lock{m_mut};
      m_cond.wait(lock, [this] {return !m_data.empty();});
      value = m_data.front();
      m_data.pop();
    }
    std::shared_ptr<T> wait_and_pop() {
      std::unique_lock<std::mutex> lock{m_mut};
      m_cond.wait(lock, [this] {return !m_data.empty();});
      std::shared_ptr<T> out = std::make_shared<T>(m_data.front());
      m_data.pop();
      return out;
    }

    bool empty() const {
      std::lock_guard<std::mutex> lock{m_mut};
      return m_data.empty();
    }
};

struct X {
  void foo(int, std::string const&);
  std::string bar(std::string const&);
};

X x;
auto f1 = std::async(&X::foo, &x, 42, "hello"); // call on pointer of x
auto f2 = std::async(&X::bar, x, "goodbye"); // call on copy of x

struct Y {
  double operator() (double);
};

Y y;
auto f3 = std::async(Y(), 3.1451); // call on moved Y of y
auto f4 = std::async(std::ref(y), 2.718); // call on y
X baz(X&);
auto f5 = std::async(baz, std::ref(x)); // call on x
class move_only {
  public:
    move_only();
    move_only(move_only&&);
    move_only(move_only const&) = delete;
    move_only& operator= (move_only&&);
    move_only& operator= (move_only const&) = delete;

    void operator() ();
};
auto f6 = std::async(move_only());

auto f7 = std::async(std::launch::async, Y(), 1.2);
auto f8 = std::async(std::launch::deferred, baz, std::ref(x));
auto f9 = std::async(
    std::launch::deferred | std::launch::async,
    baz, std::ref(x) );
auto f10 = std::async(baz, std::ref(x));



int main() {

  return 0;
}
