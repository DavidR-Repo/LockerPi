using System;
using System.Collections.Generic;
using System.Text;

namespace Network_Lock.Services
{
    public interface IEncryption
    {
        string Encrypt(string message);
        string Decrypt(string message);
    }
}
