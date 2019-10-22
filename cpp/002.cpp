#include <bits/stdc++.h>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(nullptr) {};
};


class Solution{
  public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
      int x=0, y=0, carry=0, sum=0;

      ListNode* h = nullptr, **t = &h;

      while (l1 != nullptr || l2 != nullptr) {
        x = getValueAndMoveNext(l1);
        y = getValueAndMoveNext(l2);

        sum = carry + x + y;

        ListNode* node = new ListNode(sum % 10);
        *t = node;
        t = (&node->next);

        carry = sum / 10;
      }

      if (carry > 0) {
        ListNode* node = new ListNode(carry % 10);
        *t = node;
      }

      return h;
    }

  private:
    int getValueAndMoveNext(ListNode* &l) {
      int x = 0;
      if (l != nullptr) {
        x = l->val;
        l = l->next;
      }
      return x;
    }

};

int main() {

  return 0;
}
