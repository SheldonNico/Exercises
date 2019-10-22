#include <iostream>
#include <string>

enum class MonsterT {
  Ogre, Dragon, Orc, GiantSpider, Slime,
};

struct Monster {
  MonsterT type;
  std::string name;
  int health;
};

auto monsterTname(const MonsterT& tn) -> std::string {
  switch(tn) {
    case MonsterT::Ogre: return "Ogre";
    case MonsterT::Dragon: return "Dragon";
    case MonsterT::Orc: return "Orce";
    case MonsterT::GiantSpider: return "GiantSpider";
    case MonsterT::Slime: return "Slime";
    default: return "unknown";
  }
}

auto printMonster(const Monster& m) -> void {
  std::cout << "This " << monsterTname(m.type) << " is named " <<
    m.name << " and has " << m.health << " health.\n";
}

int main () {
  Monster m1 = {MonsterT::Ogre, "Torg", 145};
  Monster m2 = {MonsterT::Slime, "Blurp", 23};

  printMonster(m1);
  printMonster(m2);
  return 0;
}
