using System;
using System.Collections.Generic;
using System.Text;

namespace Network_Lock.Services
{
    public interface IAuthentication
    {
        string Authenticate(string message);
        bool IsAuthentic(string message, IEncryption cipher);
    }
}
