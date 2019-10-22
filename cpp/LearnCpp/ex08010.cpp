#include <iostream>
#include <string>
#include <random>
#include <ctime>
#include <cstdlib>
#include <array>

class Monster {
  public:
    enum MonsterType {
      Dragon,
      Goblin,
      Ogre,
      Orc,
      Skeleton,
      Troll,
      Vampire,
      Zombie,
      Max_Monster_Types,
    };

    Monster(MonsterType mt, std::string mn, std::string mr, int mp) :
      m_type {mt}, m_name {mn}, m_roar {mr}, m_points {mp} { }
    void print() const {
      std::cout << m_name << " the " << MonsterType_to_str(m_type) << " has " << m_points << " hit points and says " << m_roar << "\n";
    }

    static Monster genMonster() {
      static const std::array<std::string, 6> names {
        "Blarg", "Moog", "Pksh", "Tyrn", "Mort", "Hans",
      };
      static const std::array<std::string, 6> roars {
        "*ROAR*", "*peep*", "*squeal*", "*whine*", "*hum*", "*burp*"
      };
      return Monster {
        static_cast<MonsterType>(getRandomNumber(0, Max_Monster_Types-1)),
        names[getRandomNumber(0, names.size()-1)],
        roars[getRandomNumber(0, roars.size()-1)],
        getRandomNumber(1, 100)
      };
    }

  private:
    MonsterType m_type;
    std::string m_name;
    std::string m_roar;
    int m_points;

    static std::string MonsterType_to_str(const MonsterType& mt) {
      switch(mt) {
        case(Dragon) : return "Dragon";
        case(Goblin) : return "Goblin";
        case(Ogre): return "Ogre";
        case(Orc): return "Orc";
        case(Skeleton): return "Skeleton";
        case(Troll): return "Troll";
        case(Vampire): return "Vampire";
        case(Zombie): return "Zombie";
        default: return "unknown";
      }
    }

    static int getRandomNumber(int min, int max) {
      static const double fraction = 1.0 / (1 + static_cast<double>(RAND_MAX));
      return static_cast<int>(rand()*fraction*(max-min+1) + min );
    }
};

int main() {
  srand(static_cast<unsigned int>(time(0)));

  Monster skele(Monster::Skeleton, "Bones", "*rattle*", 4);
  skele.print();

  Monster m = Monster::genMonster();
  m.print();

  return 0;
}
