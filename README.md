# CSharp-ChaCha20-NetStandard

Managed .Net (.NET 8) compatible [ChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) cipher written in C#

## Build status
![.NET](https://github.com/mcraiha/CSharp-ChaCha20-NetStandard/workflows/.NET/badge.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Coverage/6affaeed425241a88304a5397005c789)](https://www.codacy.com/gh/mcraiha/CSharp-ChaCha20-NetStandard/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=mcraiha/CSharp-ChaCha20-NetStandard&amp;utm_campaign=Badge_Coverage)

## Why?

Because I needed this for my personal project

## Origin

**Scott Bennett** wrote C# implementation called [ChaCha20-csharp](https://github.com/sbennett1990/ChaCha20-csharp), which works as base for my code. That is why the license is same for both projects 

## Older versions

YOu can find OLD .NET Standard and .NET 6 compatible version from [older branch](https://github.com/mcraiha/CSharp-ChaCha20-NetStandard/tree/netstandard20andnet6)

## Documentation

[Docs](https://realca.github.io/CSharp-ChaCha20-Unity/api/index.html)

## How do I use this?

Either copy the [CSChaCha20.cs](Assets/CSChaCha20Unity/CSChaCha20.cs) to your project

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

there are three different input sizes (64 bytes, 1024 bytes and 1 MiB) and comparisons are done between the original version (made by Scott Bennett) and this project

## License

All the code is licensed under [ISC License](LICENSE)
