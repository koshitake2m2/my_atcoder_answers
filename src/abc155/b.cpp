#include <bits/stdc++.h>
using namespace std;

void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n;
  cin >> n;

  bool is_ok = true;
  vector<int> a(n);
  for (auto a_i : a) {
    cin >> a_i;
    if (a_i % 2 == 0) {
      // 3でも5でも割り切れないならfalse
      if (a_i % 3 != 0 && a_i % 5 != 0) {
        is_ok = false;
      }
    }
  }
  if (is_ok) {
    cout << "APPROVED" << endl;
  } else {
    cout << "DENIED" << endl;
  }
}

int main() { solve(); }
