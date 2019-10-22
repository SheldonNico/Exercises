#include <list>
#include <mutex>
#include <algorithm>
#include <exception>
#include <memory>
#include <stack>

std::list<int> list;
std::mutex mutex;

void add_to_list(int value) {
  std::lock_guard<std::mutex> guard {mutex};
  list.push_back(value);
}

bool list_contains(int value) {
  std::lock_guard<std::mutex> guard {mutex};
  return std::find(list.begin(), list.end(), value) != list.end();
}

struct empty_stack : std::exception {
  const char* what() const throw();
};
template<typename T> class threadsafe_stack {
  private:
    std::stack<T> m_data;
    mutable std::mutex m_lock;

  public:
    threadsafe_stack() {}
    threadsafe_stack(const threadsafe_stack& other) {
      std::lock_guard<std::mutex> lock(other.m);
      m_data = other.m_data;
    }
    threadsafe_stack& operator=(const threadsafe_stack&) = delete;

    void push(T const& value) {
      std::lock_guard<std::mutex> lock(m_lock);
      m_data.push(value);
    }

    std::shared_ptr<T> pop() {
      std::lock_guard<std::mutex> lock(m_lock);
      if (m_data.empty()) throw empty_stack();
      std::shared_ptr<T> const out = std::make_shared<T>(m_data.top());
      m_data.pop();
      return out;
    }
    void pop(T& value) {
      std::lock_guard<std::mutex> lock(m_lock);
      if (m_data.empty()) throw empty_stack();
      value = m_data.top;
      m_data.pop();
    }
    bool empty() const {
      std::lock_guard<std::mutex> lock(m_lock);
      return m_data.empty();
    }
};

class Some_big_object {};
void swap(Some_big_object& lhs, Some_big_object& rhs);

class X {
  private:
    Some_big_object some_detail;
    std::mutex m_lock;

  public:
    X(Some_big_object const& sd) : some_detail{sd} {}
    friend void swap(X& lhs, X& rhs) {
      if (&lhs == &rhs) return;

      std::lock(lhs.m_lock, rhs.m_lock);
      std::lock_guard<std::mutex> lock_a{lhs.m_lock, std::adopt_lock};
      std::lock_guard<std::mutex> lock_b{rhs.m_lock, std::adopt_lock};
      swap(lhs.some_detail, rhs.some_detail);
    }
};


// hierarchical mutex
class hierarchical_mutex {
  private:
    std::mutex m_lock;
    unsigned long const m_hierarchy_value;
    unsigned long m_previous_hierarchy_value;
    static thread_local unsigned long m_this_thread_hierarcy_value;

    void check_for_hierarchy_violation() {
      if (m_this_thread_hierarcy_value <= m_hierarchy_value) {
        throw std::logic_error("mutex hierarchy violated.");
      }
    }
    void update_hierarchy_value() {
      m_previous_hierarchy_value = m_this_thread_hierarcy_value;
      m_this_thread_hierarcy_value = m_hierarchy_value;
    }

  public:
    explicit hierarchical_mutex(unsigned long const& value) :
      m_hierarchy_value{value}, m_previous_hierarchy_value{0} {}

    void lock() {
      check_for_hierarchy_violation();
      m_lock.lock();
      update_hierarchy_value();
    }
    void unlock() {
      m_this_thread_hierarcy_value = m_previous_hierarchy_value;
      m_lock.unlock();
    }
    bool try_lock() {
      check_for_hierarchy_violation();
      if (!m_lock.try_lock())
        return false;
      update_hierarchy_value();
      return true;
    }
};


hierarchical_mutex high_level_mutex{10000};
hierarchical_mutex low_level_mutex{5000};

int do_low_level_stuff();
int low_level_func() {
  std::lock_guard<hierarchical_mutex> lock(low_level_mutex);
  return do_low_level_stuff();
}

void do_high_level_stuff(int some_param);
void high_level_func() {
  std::lock_guard<hierarchical_mutex> lock(high_level_mutex);
  do_high_level_stuff(low_level_func());
}

void thread_a() {
  high_level_func();
}

hierarchical_mutex other_mutex(100);
void do_other_stuff();
void other_stuff() {
  high_level_func();
  do_other_stuff();
}

void thread_b() {
  std::lock_guard<hierarchical_mutex> lock(other_mutex);
  other_stuff();
}


int main() {

  return 0;
}
