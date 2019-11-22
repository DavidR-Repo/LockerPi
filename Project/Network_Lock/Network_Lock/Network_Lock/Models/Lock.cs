using Network_Lock.Services;
using System;
using System.Collections.Generic;
using System.Text;

namespace Network_Lock.Models
{
    
    public class Lock
    {
        public string ID { get; set; }
        public string Name { get; set; }
        public  IAuthentication Auth { get; set; }
        public IEncryption Cipher { get; set; }

    }
}
