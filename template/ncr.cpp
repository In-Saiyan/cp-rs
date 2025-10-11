#include <bits/stdc++.h>
#define int long long

const int M = 1e9 + 7;

vector<int> fact(1e6 + 1, 1);
// call prefact outside of test cases loop
void preFact() { 
    for (int i = 1; i <= 1e6; i++)
        fact[i] = (fact[i - 1] * i) % M;
}

int ncr(int n, int r) {
    if (r > n) return 0;
    return (fact[n] * modInv(fact[r]) % M * modInv(fact[n - r]) % M) % M;
}

void ARISE() {
    cout << ncr(5, 2) << "\n";
}

int32_t main() {
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    int wiz;
    cin >> wiz;
    preFact();
    while (wiz--) {
        ARISE();
    }
    return 0;
}
