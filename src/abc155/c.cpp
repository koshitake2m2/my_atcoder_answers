#include <bits/stdc++.h>
using namespace std;

void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n;
  cin >> n;
  map<string, int> mp;
  //
  string in;
  for (int i = 0; i < n; i++) {
    cin >> in;
    auto itr = mp.find(in);
    if (itr == mp.end()) {
      mp[in] = 0;
    } else {
      mp[in]++;
    }
  }
  // maxを知る
  int max_count = 0;
  for (auto itr = mp.begin(); itr != mp.end(); itr++) {
    max_count = max(max_count, itr->second);
  }
  // v == maxを出力
  for (auto itr = mp.begin(); itr != mp.end(); itr++) {
    if (max_count == itr->second) {
      cout << itr->first << endl;
    }
  }
}

int main() { solve(); }
