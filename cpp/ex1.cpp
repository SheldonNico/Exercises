#include <iostream>
#include <thread>
#include <cassert>
#include <ctime>
#include <chrono>
#include <vector>
#include <algorithm>
#include <functional>
#include <numeric>

void hello(const int& i) {
  std::cout << "thread with args " << i << " started\n";
  auto start = std::chrono::system_clock::now();

  while ( std::chrono::duration<double>(std::chrono::system_clock::now() - start).count() <= 60 ) {
    std::cout << "loop in thread " << i << "\n" ;
    std::this_thread::sleep_for(std::chrono::seconds(2));
  }

  std::cout << "thread with args " << i << " finished.\n";
}

class scoped_thread {
  private:
    std::thread m_t;

  public:
    explicit scoped_thread(std::thread t) : m_t{std::move(t)} {
      if (!m_t.joinable()) {
        throw std::logic_error("No thread.");
      }
    }
    ~scoped_thread() {
      m_t.join();
    }
    scoped_thread(scoped_thread const&) = delete;
    scoped_thread& operator=(scoped_thread const&) = delete;
};

template<typename Iterator, typename T>
class accumulate_block {
  public:
    void operator() (Iterator first, Iterator last, T& result) {
      result = std::accumulate(first, last, result);
    }
};

template<typename Iterator, typename T>
T parallel_accumulate(Iterator first, Iterator last, T init) {
  unsigned long const length = std::distance(first, last);
  if (!length) return init;

  unsigned long const min_per_thread = 25;
  unsigned long const max_threads = (length+min_per_thread-1) / min_per_thread;
  unsigned long const hardware_threads = std::thread::hardware_concurrency();
  unsigned long const num_threads = std::min( max_threads, hardware_threads==0?2:hardware_threads );

  unsigned long const block_size = length / num_threads;

  std::vector<T> results ( num_threads );
  std::vector<std::thread> threads ( num_threads-1 );

  std::cout << "using " << num_threads << "cores\n";
  for (unsigned long i {0}; i < (num_threads-1); i++) {
    Iterator block_end = first;
    std::advance(block_end, block_size);
    threads[i] = std::thread( accumulate_block<Iterator, T>(), first, block_end, std::ref(results[i]) );
    first = block_end;
  }
  accumulate_block<Iterator, T>() (first, last, results[num_threads-1]);

  std::for_each(threads.begin(), threads.end(), std::mem_fn(&std::thread::join));

  return std::accumulate(results.begin(), results.end(), init);
}


int main() {
  // move a thread
  //std::thread t_ { hello , 1 };
  //std::thread t { hello , 0 };
  //t.detach();
  //t = std::move(t_);
  //t.join();

  // use raii automatically manage thread scope.
  //scoped_thread t { std::thread{ hello, 1 } };

  // spawn threads
  //int count {0};
  //std::cout << "Give number of cores: ";
  //std::cin >> count;
  //std::vector<std::thread> threads { count };
  //for (int i {0}; i < 5; i++) {
    //threads[i] = std::thread { hello, i };
  //}

  //std::for_each(threads.begin(), threads.end(), std::mem_fn(&std::thread::join));

  // template concurrency
  //int const nums_len = 1000000;
  //std::vector<int> nums (nums_len);
  //for (int i{0}; i < nums_len; i++) nums[i] = i / 99;

  //std::cout << parallel_accumulate<std::vector<int>::iterator, int>(nums.begin(), nums.end(), 0) << "\n";

  std::thread::id master_thread;
  std::cout << master_thread << " ";
  std::cout << std::this_thread::get_id() << "\n";
  return 0;
}
