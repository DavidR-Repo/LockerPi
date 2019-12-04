using System;
using System.Net;

namespace Network_Lock.Services
{
    public interface IConnection
    {
        bool Send(string message);
        LockState Get();
    }

    public class LANConnection : IConnection
    {
        private IPAddress Address { get; set; }
        private int Port_Server { get; set; }
        private int Port_Self { get; set; }
        

        public bool Send(string message)
        {
            throw new NotImplementedException("LANConnection->Send(string message)");
        }
        public LockState Get()
        {
            throw new NotImplementedException("LANConnection->Get()");
        }



        private bool HandleSend(string message)
        {
            throw new NotImplementedException("LANConnection-> HandleSend(string message)");
        }
        private LockState HandleGet()
        {
            throw new NotImplementedException("LANConnection-> HandleGet()");
        }
    }

}
