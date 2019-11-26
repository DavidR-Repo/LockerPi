using System;
using System.Collections.Generic;

namespace Network_Lock.Models
{
    public class Permissions
    {
        private List <string> Values { get; set; }
        public IEnumerable<string> GetPermissions => Values;

        public bool Add(string permission)
        {
            if(Values == null)
                Values = new List<string>();

            bool isValid = Validate(permission);
            if (isValid)
                 Values.Add(permission);

            return isValid;
        }

        public bool Remove(string permission)
        {
            bool success = Values != null && Values.Contains(permission);
            if (success)
                Values.Remove(permission);

            return success;
        }




        private bool Validate(string permission) 
        {
            throw new NotImplementedException("Validate(string permission)");
        }


    }
}
