#include <bits/stdc++.h>
#define int long long

vector<int> sieve(int n) {
    vector<int> isPrime(n + 1, 1);
    isPrime[0] = isPrime[1] = 0;
    for (int i = 2; i * i <= n; i++) {
        if (isPrime[i]) {
            for (int j = i * i; j <= n; j += i)
                isPrime[j] = 0;
        }
    }
    return isPrime;
}

void ARISE() {
    // code here
}

int32_t main() {
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    int wiz;
    cin >> wiz;
    sieve(1e7); // call here
    while (wiz--) {
        ARISE();
    }
    return 0;
}

