#include <bits/stdc++.h>
using namespace std;

char change(char c, int n) {
  char new_c;
  int diff = (int)('Z' - c);
  if (diff >= n) {
    new_c = (char)(c + n);
  } else {
    new_c = (char)('A' - 1 + (n - diff));
  }
  return new_c;
}

void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n;
  string str;
  cin >> n >> str;

  for (int i = 0; i < str.size(); i++) {
    str[i] = change(str[i], n);
  }
  cout << str << endl;
}

int main() { solve(); }
