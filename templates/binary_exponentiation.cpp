// Algo: Binary Exponentiation (Used to (a^b) mod m in log(b) time)
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

int32_t main()
{
    // Example
    int a,b;
    cin >> a >> b;
    cout << bin_exp(a,b);
    return 0;
}