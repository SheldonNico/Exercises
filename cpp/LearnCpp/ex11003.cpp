#include <iostream>
#include <string>
#include <cmath>
#include <cstdlib>

class Creature {
  protected:
    std::string m_name;
    char m_symbol;
    int m_health, m_damage, m_gold;

  public:
    Creature(std::string name, char symbol, int health, int damage, int gold) :
      m_name { name }, m_symbol { symbol }, m_health {health}, m_damage {damage}, m_gold {gold} {}

    std::string getName() { return m_name; }
    char getSymbol() { return m_symbol; }
    int getHealth() { return m_health; }
    int getDamage() { return m_damage; }
    int getGold() { return m_gold; }

    void reduceHealth(const int& h) { m_health -= h; }
    void addGold(const int& g) { m_gold += g; }
    bool isDead() const { return (m_health <= 0); }
};
class Monster;
class Player;

int getRandomInt(const int& min, const int& max) {
  static const double fraction = 1 / (static_cast<double>(RAND_MAX) + 1.0);
  return static_cast<int>(rand() * fraction * (max - min + 1)) + min;
}

class Monster : public Creature {
  public:
    enum Type {
      DRAGON, ORC, SLIME, MAX_TYPES
    };
    struct MonsterData {
      std::string name;
      char symbol;
      int health, damage, gold;
    };
    static MonsterData monsterData[MAX_TYPES];

    Monster(const Type& mt) :
      Creature(monsterData[mt].name, monsterData[mt].symbol, monsterData[mt].health, monsterData[mt].damage, monsterData[mt].gold ) { }

    static Monster getRandomMonster() {
      return static_cast<Type>(getRandomInt(0, 2));
    }

    void attackPlayer(Player& p);
  private:
};

class Player : public Creature {
  private:
    int m_level{1};

  public:
    Player(std::string name, int level=1) : Creature(name, '@', 10, 1, 0), m_level {level}  {}
    void levelUp() {
      m_damage += 1;
      m_level += 1;
    }

    int getLevel() { return m_level; }
    bool hasWon() const { return bool(); }
    operator bool() const { return (m_level >= 0); }


    void fightMonster(Monster& m);
    void attackMonster(Monster& m);
};


Monster::MonsterData Monster::monsterData[Monster::MAX_TYPES] {
  { "dragon", 'D', 20, 4, 100 },
  { "orc", 'o', 4, 2, 25 },
  { "slime", 's', 1, 1, 10 }
};

void Monster::attackPlayer(Player& p) {
  p.reduceHealth( m_damage );
  std::cout << "The " << m_name << " hit you for " << m_damage << " damage\n";
}
void Player::attackMonster(Monster& m) {
  m.reduceHealth( m_damage );
  std::cout << "You hit the " << m.getName() << " for " << m_damage << " damage\n";
}

void Player::fightMonster(Monster& m) {
  char op = ' ';

  do {
    do {
      std::cout << "(R)un or (F)ight: ";
      std::cin >> op;
      if (std::cin.fail()) {
        std::cin.clear();
      }
      std::cin.ignore(32767, '\n');
    } while ( !(op == 'f' || op == 'r' || op == 'F' || op == 'R') );

    if (op == 'r' || op == 'R') {
      if (getRandomInt(0, 1) > 0) {
        std::cout << "You failed to flee.\n";
        m.attackPlayer(*this);
      } else {
        std::cout << "You successfully fled.\n";
        break;
      }
    } else {
      this->attackMonster(m);
      m.attackPlayer(*this);
    }
  } while (!m.isDead() && !isDead());

  if (m.isDead()) {
    std::cout << "the " << m.getName() << " is killed, you leveled up!\n";
    addGold(m.getGold());
    levelUp();
  }
}
int main()
{
  srand(static_cast<unsigned int>(time(0)));

  //Creature o("orc", 'o', 4, 2, 10);
  //o.addGold(5);
  //o.reduceHealth(1);
  //std::cout << "The " << o.getName() << " has " << o.getHealth() << " health and is carrying " << o.getGold() << " gold.\n";

  //std::cout << "Enter your name: ";
  //std::string name; std::cin >> name;
  //Player p {name};
  //std::cout << "Welcome, " << p.getName() << ".\n";
  //std::cout << "You have " << p.getHealth() << " health and are carrying " << p.getGold() << " gold.\n";

  //Monster m(Monster::ORC);
  //std::cout << "A " << m.getName() << " (" << m.getSymbol() << ") was created.\n";

  //for (int i = 0; i < 10; ++i)
  //{
  //Monster m = Monster::getRandomMonster();
  //std::cout << "A " << m.getName() << " (" << m.getSymbol() << ") was created.\n";
  //}


  std::cout << "Enter your name: ";
  std::string name; std::cin >> name;
  Player p {name};
  std::cout << "Welcome, " << p.getName() << ".\n";

  do {
    Monster m = Monster::getRandomMonster();
    std::cout << "You have encountered a " << m.getName() << " (" << m.getSymbol() << ")\n";
    p.fightMonster(m);
  } while ( !p.isDead() && !p.hasWon() );

  if (p.hasWon()) {
    std::cout << "You win at level " << p.getLevel() << " and with " << p.getGold() << " gold.\n";

  } else {
    std::cout << "You died at level " << p.getLevel() << " and with " << p.getGold() << " gold.\n";
    std::cout << "Too bad you can't take it with you!";
  }

  return 0;
}
