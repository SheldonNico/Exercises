#include <iostream>
#include <string>

enum class Animal {
  Pig, Chicken, Goat, Cat, Dog, Ostrich,
};

auto getAnimalName(const Animal& an) -> std::string {
  switch (an) {
    case Animal::Pig: return "pig";
    case Animal::Chicken: return "chicken";
    case Animal::Goat: return "goat";
    case Animal::Dog: return "dog";
    case Animal::Ostrich: return "ostrich";
    default: return "unknown";
  }
}

auto getAnimalLeg(const Animal& an) -> int {
  switch (an) {
    case Animal::Chicken:
    case Animal::Ostrich:
      return 2;
    case Animal::Goat:
    case Animal::Dog:
    case Animal::Pig:
      return 4;
    default: return -1;
  }
}

int main() {
  Animal pig = Animal::Pig, chicken = Animal::Chicken;

  std::cout << "A " << getAnimalName(pig) << " has " << getAnimalLeg(pig) << " legs\n";
  std::cout << "A " << getAnimalName(chicken) << " has " << getAnimalLeg(chicken) << " legs\n";
  return 0;
}
