#include <bits/stdc++.h>
using namespace std;


void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<int> x(n);
  for (auto& x_i: x) {
      cin >> x_i;
  }
  int min_tairyoku = 1e9;
  for (int p = 1; p <= 100; p++) {
      int sum_tairyoku = 0;
      for (int i = 0; i < n; i++) {
          sum_tairyoku += (x[i] - p) * (x[i] - p);
      }
      min_tairyoku = min(min_tairyoku, sum_tairyoku);
  }
  cout << min_tairyoku << endl;
}

int main() {
  solve();
}
