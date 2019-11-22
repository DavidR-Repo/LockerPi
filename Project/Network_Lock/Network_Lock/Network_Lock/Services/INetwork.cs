using System;
using System.Collections.Generic;
using System.Text;

namespace Network_Lock.Services
{
    public interface INetwork
    {
        bool SendMessage(string message);
        string GetMessage();
    }
}
