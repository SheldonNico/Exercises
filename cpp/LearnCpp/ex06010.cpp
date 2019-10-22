#include <iostream>

enum class Item {
  Potion, Torch, Arrow,
};

struct Player {
  int* stock;
};

int countTotalItems(const Player& p) {
  return p.stock[0] + p.stock[1] + p.stock[2];
}

int main() {
  Player p {};
  p.stock = new int[3] {2, 4, 10};
  std::cout << "the player has " << countTotalItems(p) << "items\n";

  delete [] p.stock;
}
