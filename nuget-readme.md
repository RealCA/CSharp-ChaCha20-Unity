# CSharp-ChaCha20-NetStandard

Managed .Net (Standard 2.0 and .NET 6) compatible [ChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) cipher written in C#

## Documentation

[Docs](https://mcraiha.github.io/CSharp-ChaCha20-NetStandard/api/index.html)

## How do I use this?

```csharp
using CSChaCha20;

byte[] mySimpleTextAsBytes = Encoding.ASCII.GetBytes("Plain text I want to encrypt");

// Do NOT use these key and nonce values in your own code!
byte[] key = new byte[32] { 142, 26, 14, 68, 43, 188, 234, 12, 73, 246, 252, 111, 8, 227, 57, 22, 168, 140, 41, 18, 91, 76, 181, 239, 95, 182, 248, 44, 165, 98, 34, 12 };
byte[] nonce = new byte[12] { 139, 164, 65, 213, 125, 108, 159, 118, 252, 180, 33, 88 };
uint counter = 1;

// Encrypt
ChaCha20 forEncrypting = new ChaCha20(key, nonce, counter);
byte[] encryptedContent = new byte[mySimpleTextAsBytes.Length];
forEncrypting.EncryptBytes(encryptedContent, mySimpleTextAsBytes);

// Decrypt
ChaCha20 forDecrypting = new ChaCha20(key, nonce, counter);
byte[] decryptedContent = new byte[encryptedContent.Length];
forDecrypting.DecryptBytes(decryptedContent, encryptedContent);

```

You can try out the code in [.NET Fiddle](https://dotnetfiddle.net/4D6E5Z)