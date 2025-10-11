// Algo: Mod Inverse of a number using Bin_Exp
#include <bits/stdc++.h>
using namespace std;
#define int long long
const int mod=1e9+7;

int bin_exp(int a, int b)
{
    int res=1;
    a%mod;
    while(b)
    {
        if(b%2==1) res=(res*a)%mod;
        a=(a*a)%mod;
        b/=2;
    }
    return res;
}
// Time Complexity: O(logn)

// Modular inverse using Fermat's Little Theorem
int mod_inverse(int n)
{
    return bin_exp(n,mod-2);
}

int32_t main()
{
    // Example
    int n;
    cin >> n;
    cout << mod_inverse(n) << endl;
    // Also used for modular division
    int a,b;
    cin >> a >> b;
    cout << (a%mod*mod_inverse(b)%mod)%mod << endl;
    return 0;
}