#include <thread>
#include <future>
#include <iostream>
#include <chrono>
#include <string>

std::string fetchDataFromDB(std::string recvData) {
  std::this_thread::sleep_for(std::chrono::seconds(5));

  return "DB_" + recvData;
}

std::string fetchDataFromFile(std::string recvData) {
  std::this_thread::sleep_for(std::chrono::seconds(5));

  return "File_" + recvData;
}

int main() {
  std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
  std::future<std::string> resultFromDB = std::async(std::launch::async, fetchDataFromDB, "Data");
  std::future<std::string> resultFromFile = std::async(std::launch::async, fetchDataFromFile, "Data");

  std::string dbData = resultFromDB.get();
  std::string fileData = resultFromFile.get();
  auto end = std::chrono::system_clock::now();
  auto diff = std::chrono::duration_cast<std::chrono::seconds> (end-start).count();
  std::cout << "Total time token = " << diff << " seconds" << std::endl;

  std::cout << "Data => " << dbData + "::" + fileData << std::endl;
  return 0;
}
