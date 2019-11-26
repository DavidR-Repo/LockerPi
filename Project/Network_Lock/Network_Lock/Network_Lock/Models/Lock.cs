

namespace Network_Lock.Models
{
    public class IService { }
    public class Lock
    {
        public string ID { get; set; }
        public string Name { get; set; }
        public  Permissions Permissions { get; set; }
        public IService Service { get; set; }

    }
}
