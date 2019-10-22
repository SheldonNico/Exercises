#include <thread>
#include <iostream>
#include <vector>
#include <algorithm>
#include <mutex>

void thread_function() {
  for (int i{0}; i < 10000; i++) {
    std::cout << "thread function executing: " << i << std::endl;
  }
}

class threa_functor {
  public:
    void operator() () {
      for (int i{0}; i < 10000; i++) {
        std::cout << "thread functor executing: " << i << std::endl;
      }
    }


};

class ThreadRAII {
  private:
    std::thread& m_thread;

  public:
    ThreadRAII(std::thread& threadObj) : m_thread { threadObj } { }

    ~ThreadRAII() {
      if (m_thread.joinable())
        m_thread.join();
    }
};

void threadCallback(int x, std::string str) {
  std::cout << "passed  number is " << x << std::endl;
  std::cout << "passed string is " << str << std::endl;
}

void threadRef(int const& x) {
  int& y = const_cast<int&>(x);
  y++;
  std::cout << "Inside thread x = " << x << std::endl;

}

class DummyClass {
  public:
    DummyClass() {}
    DummyClass(DummyClass const& obj) {}
    void memberfunc(int x) {
      std::cout << "Inside class member function " << x << std::endl;
    }

};

class Wallet {
  private:
    int m_money;
    std::mutex m_lock;

  public:
    Wallet() : m_money{0} {}
    int get_money() { return m_money; }
    void addMoney(int money ) {
      std::lock_guard<std::mutex> lock{m_lock};

      for (int i{0}; i < money; i++) {
        m_money++;
      }
    }
};

int testMultithreadWallet() {
  Wallet wallet;
  std::vector<std::thread> threads;
  for (int i{0}; i < 5; i++) {
    threads.push_back(std::thread(&Wallet::addMoney, &wallet, 10000));
  }

  for (auto& t : threads)
    t.join();

  return wallet.get_money();
}


int main() {
  //std::thread threadObj{thread_function};
  //std::thread threadObj{threa_functor()};
  //std::thread threadObj{[] {
    //for (int i {0}; i < 100; i++) {
      ////std::cout << "thread functor executing: " << i << std::endl;
    //}
    //std::cout << "child thread id: " << std::this_thread::get_id() << std::endl;
  //}};

  //ThreadRAII wrapper { threadObj };

  //threadObj.join();
  //std::cout << "parent thread id: " << std::this_thread::get_id() << std::endl;

  //int x = 10; std::string str = "Sample string";
  //std::thread threadObj(threadCallback, x, str);
  //threadObj.join();

  //int x = 9;
  //std::cout << "In main thread: before thread start x = " << x << std::endl;
  //std::thread threadObj(threadRef, std::ref(x));
  //threadObj.join();
  //std::cout << "In main thread: after thread joins x = " << x << std::endl;

  //DummyClass dummyObj;
  //int x = 10;
  ////wrong std::thread threadObj(dummyObj.memberfunc, x);
  //std::thread threadObj(&DummyClass::memberfunc, &dummyObj, x);
  //threadObj.join();
  for (int k{0}; k < 1000; k++) {
    int val = testMultithreadWallet();
    if (val != 10000*5) {
      std::cout << "Error at count = " << k << " money in wallet is " << val << std::endl;
    }
  }

  return 0;
}
