#include <iostream>
#include <array>
#include <utility>
#include <random>
#include <ctime>

enum class Rank {
  Two, Three, Four, Five, Six, Seven,
  Eight, Nine, Ten, Jack, Queen, King, Ace
};

enum class Suit {
  Club, Diamond, Heart, Spade
};

struct Card {
  Rank r;
  Suit s;
};

std::string rank_str(const Rank& r) {
  switch (r) {
    case (Rank::Two):   return "2";
    case (Rank::Three): return "3";
    case (Rank::Four):  return "4";
    case (Rank::Five):  return "5";
    case (Rank::Six):   return "6";
    case (Rank::Seven): return "7";
    case (Rank::Eight): return "8";
    case (Rank::Nine):  return "9";
    case (Rank::Ten):   return "10";
    case (Rank::Jack):  return "J";
    case (Rank::Queen): return "Q";
    case (Rank::King):  return "K";
    case (Rank::Ace):   return "1";
    default: return "unknown";
  }
}

std::string suit_str(const Suit& s) {
  switch (s) {
    case (Suit::Club) : return "C";
    case (Suit::Diamond) : return "D";
    case (Suit::Heart) : return "H";
    case (Suit::Spade) : return "S";
    default: return "unknown";
  }
}

void printCard(const Card& c) {
  std::cout << rank_str(c.r) << suit_str(c.s);
}

void printDeck( const std::array<Card, 52>& deck  ) {
  for (const auto& c : deck) {
    printCard(c);
    std::cout << " ";
  }
  std::cout << "\n";
}

void swapCard(Card& c1, Card& c2) {
  std::swap( c1.r, c2.r );
  std::swap( c1.s, c2.s );
}

namespace _RandomSeed {
  std::mt19937 mersenne { static_cast<std::mt19937::result_type>(std::time(nullptr)) };
}

int random_int(const int& st, const int& et) {
  std::uniform_int_distribution<> rand(st, et);
  return rand(_RandomSeed::mersenne);
}

void shuffleDeck( std::array<Card, 52>& deck ) {
  using index_t = std::array<Card, 52>::size_type;
  index_t pick {0};
  for (auto c : deck) {
    pick = random_int(0, 51);
    swapCard(c, deck[pick]);
  }
}

int getCardValue(const Card& c) {
  switch (c.r) {
    case (Rank::Two):   return 2;
    case (Rank::Three): return 3;
    case (Rank::Four):  return 4;
    case (Rank::Five):  return 5;
    case (Rank::Six):   return 6;
    case (Rank::Seven): return 7;
    case (Rank::Eight): return 8;
    case (Rank::Nine):  return 9;
    case (Rank::Ten):   return 10;
    case (Rank::Jack):  return 10;
    case (Rank::Queen): return 10;
    case (Rank::King):  return 10;
    case (Rank::Ace):   return 11;
    default: return 0;
  }
}

int main() {
  std::array<Card, 52> cards {};
  int num_of_rank = 13, num_of_suit = 4;
  for (int r {0}; r < num_of_rank; ++r) {
    for (int s {0}; s < num_of_suit; ++s) {
      cards[s*num_of_rank+r] = {
        static_cast<Rank>(r),
        static_cast<Suit>(s),
      };
    }
  }

  std::cout << "Before shuffle the deck: ";
  printDeck(cards);
  std::cout << "After shuffle the deck: ";
  shuffleDeck(cards);
  printDeck(cards);

  return 0;
}
