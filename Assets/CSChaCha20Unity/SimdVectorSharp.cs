using System;
using System.Runtime.InteropServices;

namespace CSChaCha20Unity.SimdVectorSharp
{
    public static class SimdNative
    {
        const string DLL = "SimdVector";

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern bool Vector128_IsHardwareAccelerated();
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern bool Vector256_IsHardwareAccelerated();
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern bool Vector512_IsHardwareAccelerated();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern void Vector128_Create(byte[] input, UIntPtr length, UIntPtr offset, byte[] outArray);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern void Vector128_Xor(byte[] a, byte[] b, byte[] outArray);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern void Vector256_Create(byte[] input, UIntPtr length, UIntPtr offset, byte[] outArray);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern void Vector256_Xor(byte[] a, byte[] b, byte[] outArray);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern void Vector512_Create(byte[] input, UIntPtr length, UIntPtr offset, byte[] outArray);
        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)]
        public static extern void Vector512_Xor(byte[] a, byte[] b, byte[] outArray);
    }

    // ---------------------- VECTOR STRUCTS ----------------------
    public unsafe struct Vector128
    {
        public fixed byte Data[16];

        public static Vector128 Create(byte[] input, int offset)
        {
            if (offset < 0 || offset + 16 > input.Length)
                throw new ArgumentOutOfRangeException(nameof(offset));

            Vector128 v = new Vector128();
            byte[] tmp = new byte[16];
            SimdNative.Vector128_Create(input, (UIntPtr)input.Length, (UIntPtr)offset, tmp);

            for (int i = 0; i < 16; i++) v.Data[i] = tmp[i];
            return v;
        }

        public static Vector128 operator ^(Vector128 a, Vector128 b)
        {
            Vector128 result = new Vector128();
            byte[] tmpA = new byte[16];
            byte[] tmpB = new byte[16];
            byte[] tmpR = new byte[16];

            for (int i = 0; i < 16; i++) { tmpA[i] = a.Data[i]; tmpB[i] = b.Data[i]; }
            SimdNative.Vector128_Xor(tmpA, tmpB, tmpR);
            for (int i = 0; i < 16; i++) result.Data[i] = tmpR[i];

            return result;
        }

        public void CopyTo(byte[] output, int offset)
        {
            if (offset < 0 || offset + 16 > output.Length)
                throw new ArgumentOutOfRangeException(nameof(offset));

            for (int i = 0; i < 16; i++) output[offset + i] = Data[i];
        }

        public static bool IsHardwareAccelerated => SimdNative.Vector128_IsHardwareAccelerated();
    }

    public unsafe struct Vector256
    {
        public fixed byte Data[32];

        public static Vector256 Create(byte[] input, int offset)
        {
            if (offset < 0 || offset + 32 > input.Length)
                throw new ArgumentOutOfRangeException(nameof(offset));

            Vector256 v = new Vector256();
            byte[] tmp = new byte[32];
            SimdNative.Vector256_Create(input, (UIntPtr)input.Length, (UIntPtr)offset, tmp);

            for (int i = 0; i < 32; i++) v.Data[i] = tmp[i];
            return v;
        }

        public static Vector256 operator ^(Vector256 a, Vector256 b)
        {
            Vector256 result = new Vector256();
            byte[] tmpA = new byte[32];
            byte[] tmpB = new byte[32];
            byte[] tmpR = new byte[32];

            for (int i = 0; i < 32; i++) { tmpA[i] = a.Data[i]; tmpB[i] = b.Data[i]; }
            SimdNative.Vector256_Xor(tmpA, tmpB, tmpR);
            for (int i = 0; i < 32; i++) result.Data[i] = tmpR[i];

            return result;
        }

        public void CopyTo(byte[] output, int offset)
        {
            if (offset < 0 || offset + 32 > output.Length)
                throw new ArgumentOutOfRangeException(nameof(offset));

            for (int i = 0; i < 32; i++) output[offset + i] = Data[i];
        }

        public static bool IsHardwareAccelerated => SimdNative.Vector256_IsHardwareAccelerated();
    }

    public unsafe struct Vector512
    {
        public fixed byte Data[64];

        public static Vector512 Create(byte[] input, int offset)
        {
            if (offset < 0 || offset + 64 > input.Length)
                throw new ArgumentOutOfRangeException(nameof(offset));

            Vector512 v = new Vector512();
            byte[] tmp = new byte[64];
            SimdNative.Vector512_Create(input, (UIntPtr)input.Length, (UIntPtr)offset, tmp);

            for (int i = 0; i < 64; i++) v.Data[i] = tmp[i];
            return v;
        }

        public static Vector512 operator ^(Vector512 a, Vector512 b)
        {
            Vector512 result = new Vector512();
            byte[] tmpA = new byte[64];
            byte[] tmpB = new byte[64];
            byte[] tmpR = new byte[64];

            for (int i = 0; i < 64; i++) { tmpA[i] = a.Data[i]; tmpB[i] = b.Data[i]; }
            SimdNative.Vector512_Xor(tmpA, tmpB, tmpR);
            for (int i = 0; i < 64; i++) result.Data[i] = tmpR[i];

            return result;
        }

        public void CopyTo(byte[] output, int offset)
        {
            if (offset < 0 || offset + 64 > output.Length)
                throw new ArgumentOutOfRangeException(nameof(offset));

            for (int i = 0; i < 64; i++) output[offset + i] = Data[i];
        }

        public static bool IsHardwareAccelerated => SimdNative.Vector512_IsHardwareAccelerated();
    }
}
