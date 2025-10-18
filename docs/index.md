# CSharp-ChaCha20-NetStandard

Managed .Net (.NET 6) unity compatible [ChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) cipher written in C# and using rust code to replace unsupported library System.Runtime.Intrinsics in orginal code (https://github.com/mcraiha/CSharp-ChaCha20-NetStandard/blob/58b7bd5f0774b304fb580ed4435e6789613a6e70/src/CSChaCha20.cs#L21C1-L21C33)

## GitHub
[CSharp-ChaCha20-Unity](https://github.com/RealCA/CSharp-ChaCha20-Unity)

## Documentation
[Documentation](api/) 

## How do I use this?
Either copy the [CSChaCha20.cs](https://github.com/RealCA/CSharp-ChaCha20-Unity/blob/master/Assets/CSChaCha20Unity/CSChaCha20Unity.cs) to your project or use [LibChaCha20](https://www.nuget.org/packages/LibChaCha20/) nuget package

Then do code like
```csharp
using CSChaCha20Unity;
using System;
using System.Diagnostics;
using System.IO;
using UnityEngine;
using Debug = UnityEngine.Debug;

public class ChaCha20Test : MonoBehaviour
{
    [Header("Assign a text file in the Unity Inspector")]
    public TextAsset textFile;
    ChaCha20 encryptor;


    // Do NOT use these key/nonce in production
    byte[] key = new byte[32] {
            142, 26, 14, 68, 43, 188, 234, 12,
            73, 246, 252, 111, 8, 227, 57, 22,
            168, 140, 41, 18, 91, 76, 181, 239,
            95, 182, 248, 44, 165, 98, 34, 12
        };

    byte[] nonce = new byte[12] { 139, 164, 65, 213, 125, 108, 159, 118, 252, 180, 33, 88 };

    uint counter = 1;

    [ContextMenu("Test 1")]
    void Test1()
    {
        // Example text to encrypt
        var watch = new Stopwatch();
        watch.Start();
        byte[] plainTextBytes = textFile.GetData<byte>().ToArray();

        if (encryptor == null)
            encryptor = new ChaCha20(key);

        //show number of bytes
        Debug.Log($"Testing ChaCha20 in Unity for {plainTextBytes.Length} bytes");
        Debug.Log($"[Unity] Plain bytes: {BitConverter.ToString(plainTextBytes)}");
        var lap = watch.ElapsedMilliseconds;
        // Encrypt
        byte[] encryptedBytes = new byte[plainTextBytes.Length];
        encryptor.EncryptBytes(encryptedBytes, plainTextBytes, nonce, counter);
        Debug.Log($"[Unity] Time to encrypt: {watch.ElapsedMilliseconds - lap} ms");

        Debug.Log($"[Unity] Encrypted bytes: {BitConverter.ToString(encryptedBytes)}");

        lap = watch.ElapsedMilliseconds;
        // Decrypt
        byte[] decryptedBytes = new byte[encryptedBytes.Length];
        encryptor.DecryptBytes(decryptedBytes, encryptedBytes, nonce, counter);
        Debug.Log($"[Unity] Time to decrypt: {watch.ElapsedMilliseconds - lap} ms");

        Debug.Log($"[Unity] Decrypted bytes: {BitConverter.ToString(decryptedBytes)}");
        // Path to your output file
        string outputPath = Path.Combine(Application.persistentDataPath, "decrypted_output.bin");

        // Ensure directory exists
        Directory.CreateDirectory(Path.GetDirectoryName(outputPath)!);

        // Write bytes to file (overwrite if exists)
        File.WriteAllBytes(outputPath, decryptedBytes);

        Debug.Log($"[Unity] Decrypted bytes written to: {outputPath}");
        watch.Stop();
        Debug.Log($"[Unity] Time elapsed: {watch.ElapsedMilliseconds} ms");
    }

    [ContextMenu("Test 2")]
    void Test2()
    {
        // Example text to encrypt
        var watch = new Stopwatch();
        watch.Start();
        byte[] plainTextBytes = textFile.GetData<byte>().ToArray();

        // Do NOT use these key/nonce in production
        byte[] key = new byte[32] {
            142, 26, 14, 68, 43, 188, 234, 12,
            73, 246, 252, 111, 8, 227, 57, 22,
            168, 140, 41, 18, 91, 76, 181, 239,
            95, 182, 248, 44, 165, 98, 34, 12
        };

        byte[] nonce = new byte[12] { 139, 164, 65, 213, 125, 108, 159, 118, 252, 180, 33, 88 };
        uint counter = 1;
        //show number of bytes
        Debug.Log($"Testing ChaCha20 in Unity for {plainTextBytes.Length} bytes");
        Debug.Log($"[Unity] Plain bytes: {BitConverter.ToString(plainTextBytes)}");
        var lap = watch.ElapsedMilliseconds;
        // Encrypt
        ChaCha20 encryptor = new ChaCha20(key, nonce, counter);
        byte[] encryptedBytes = new byte[plainTextBytes.Length];
        encryptor.EncryptBytes(encryptedBytes, plainTextBytes);
        Debug.Log($"[Unity] Time to encrypt: {watch.ElapsedMilliseconds - lap} ms");

        Debug.Log($"[Unity] Encrypted bytes: {BitConverter.ToString(encryptedBytes)}");

        lap = watch.ElapsedMilliseconds;
        // Decrypt
        ChaCha20 decryptor = new ChaCha20(key, nonce, counter);
        byte[] decryptedBytes = new byte[encryptedBytes.Length];
        decryptor.DecryptBytes(decryptedBytes, encryptedBytes);
        Debug.Log($"[Unity] Time to decrypt: {watch.ElapsedMilliseconds - lap} ms");

        Debug.Log($"[Unity] Decrypted bytes: {BitConverter.ToString(decryptedBytes)}");
        // Path to your output file
        string outputPath = Path.Combine(Application.persistentDataPath, "decrypted_output.bin");

        // Ensure directory exists
        Directory.CreateDirectory(Path.GetDirectoryName(outputPath)!);

        // Write bytes to file (overwrite if exists)
        File.WriteAllBytes(outputPath, decryptedBytes);

        Debug.Log($"[Unity] Decrypted bytes written to: {outputPath}");
        watch.Stop();
        Debug.Log($"[Unity] Time elapsed: {watch.ElapsedMilliseconds} ms");
    }
}


```

## License

All the code is licensed under [ISC License](https://github.com/RealCA/CSharp-ChaCha20-Unity/blob/master/LICENSE)
