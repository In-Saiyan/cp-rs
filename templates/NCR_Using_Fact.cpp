// Algo: NCR mod m Using factorial vector (for q queries it's beneficial as after building fact vector once we get ncr in O(1))
#include <bits/stdc++.h>
using namespace std;
#define int long long
const int mod=1e9+7;
const int N=1e7;

// Helper functions
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

int mod_inverse(int n)
{
    return bin_exp(n,mod-2);
}

vector<int> fact(N+10);
int ncr(int n,int r)
{
    if(r<0 || r>n) return 0;
    if(r==0 || r==n) return 1;
    int ret;
    ret=(fact[n]%mod*mod_inverse(fact[r])%mod)%mod;
    ret=(ret%mod*mod_inverse(fact[n-r])%mod)%mod;
    return ret;
}

int32_t main()
{
    // Building fact vector first
    for(int i=1;i<=1e7+10;i++)
        fact[i]=((i==0)?1:((fact[i-1]%mod*i%mod)%mod));

    // Example
    int n,r;
    cin >> n >> r;
    cout << ncr(n,r) << endl;
}