using Aki.Reflection.Patching;
using System;
using System.Reflection;
using System.Linq;

namespace TarkovStash.Patches
{

    internal class NotificationManagerClassPatch : ModulePatch
    {

        private const BindingFlags AllBindingFlags = BindingFlags.Instance | BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic;


        protected override MethodBase GetTargetMethod()
        {
            Type targetType = typeof(NotificationManagerClass);
            Type returnType = GetNestedType(targetType, "EProcessError");
            Type[] parameterTypes = { typeof(string) };

            return GetMethod(targetType, returnType, parameterTypes);
        }

        private Type GetNestedType(Type targetType, string nestedTypeName)
        {
            return targetType.GetNestedTypes(AllBindingFlags)
                                 .FirstOrDefault(t => t.Name == nestedTypeName);
        }

        public static MethodInfo GetMethod(Type targetType, Type returnType, Type[] parameterTypes)
        {
            return targetType.GetMethods(AllBindingFlags)
                                 .FirstOrDefault(m => m.ReturnType == returnType &&
                                                      m.GetParameters().Select(p => p.ParameterType).SequenceEqual(parameterTypes));
        }


        [PatchPostfix]
        public static void PatchPostfix(string message)
        {
            Plugin.Instance.handleMessage(message);
        }

    }

}
