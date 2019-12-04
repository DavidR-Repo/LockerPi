using System;
using System.Collections.Generic;

namespace Network_Lock.Services
{
    public enum LockState { LOCKED, UNLOCKED, ERROR }

    public interface IService
    {
        bool SendLock();
        bool SendUnlock();
        LockState GetLockState();
        IEnumerable<string> GetPermissions();
    }

    public class PrototypeService : IService
    {
        public LockState State { get; set; }
        public IEnumerable<string> PermissionsList {get; set;}
        public bool SendLock()
        {
            Console.WriteLine("Sent 'SendLock' signal");
            return true;
        }
        public bool SendUnlock()
        {
            Console.WriteLine("Sent 'SendUnlock' signal");
            return true;
        }
        public LockState GetLockState()
        {
            Console.WriteLine("Sent 'GetLockState' signal");
            Console.WriteLine("Waiting for 'LockState'");
            return State;
        }
        public IEnumerable<string> GetPermissions()
        {
            Console.WriteLine("Sent 'GETPERMISSIONS' signal");
            return PermissionsList;
        }
    }

    public class SecureService : IService
    {
        public IConnection Connection { get; set; }
        public IEnumerable<string> PermissionsList { get; set; }
        public bool SendLock()
        {
            Console.WriteLine("Sent 'SendLock' signal");
            return HandleSendLock();
        }
        public bool SendUnlock()
        {
            Console.WriteLine("Sent 'SendUnlock' signal");
            return HandleSendUnlock();
        }
        public LockState GetLockState()
        {
            Console.WriteLine("Sent 'GetLockState' signal");
            Console.WriteLine("Waiting for 'LockState'");
            return HandleGetLockState();
        }
        public IEnumerable<string> GetPermissions()
        {
            Console.WriteLine("Sent 'GETPERMISSIONS' signal");
            return HandleGetPermissions();
        }


        private bool Authenticate(string message)
        {
            throw new NotImplementedException("Authenticate(string message)");
        }
        private String Encrypt(string message)
        {
            throw new NotImplementedException("Encrypt(string message)");
        }
        private String Decrypt(string message)
        {
            throw new NotImplementedException("Decrypt(string message)");
        }
        private bool Acknowledge()
        {
            throw new NotImplementedException("Acknowledge()");
        }


        private bool HandleSendLock()
        {
            throw new NotImplementedException("HandleSendLock()");
        }

        private bool HandleSendUnlock()
        {
            throw new NotImplementedException("HandleSendUnlock()");
        }

        private LockState HandleGetLockState()
        {
            throw new NotImplementedException("HandleLockState()");
        }

        private IEnumerable<string> HandleGetPermissions()
        {
            throw new NotImplementedException("HandleGetPermissions()");
        }
    }
}
