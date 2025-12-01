import { Link } from "react-router";
import { Button } from "~/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "~/components/ui/avatar";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "~/components/ui/navigation-menu";
import type { Auth0User } from "~/types/auth";

interface NavbarProps {
  user?: Auth0User;
}

export function Navbar({ user }: NavbarProps) {
  const getInitials = (name?: string) => {
    if (!name) return "?";
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  };

  return (
    <header className="sticky top-0 z-50 w-full border-b border-primary/50 bg-zama-500 backdrop-blur supports-[backdrop-filter]:bg-zama-500/60">
      <div className="container mx-auto flex h-16 max-w-screen-2xl items-center justify-between">
        {/* Logo */}
        <div className="flex items-center gap-6">
          <Link to="/" className="flex items-center space-x-2">
            <img src="/zama-logo.svg" alt="Zama Logo" className="h-8 w-auto" />
          </Link>

          {/* Navigation Menu */}
          <NavigationMenu className="hidden md:flex">
            <NavigationMenuList>
              <NavigationMenuItem>
                <NavigationMenuLink
                  asChild
                  className={navigationMenuTriggerStyle()}
                >
                  <Link to="/pricing">Pricing</Link>
                </NavigationMenuLink>
              </NavigationMenuItem>
              {user && (
                <>
                  <NavigationMenuItem>
                    <NavigationMenuLink
                      asChild
                      className={navigationMenuTriggerStyle()}
                    >
                      <Link to="/dashboard">Dashboard</Link>
                    </NavigationMenuLink>
                  </NavigationMenuItem>
                  <NavigationMenuItem>
                    <NavigationMenuLink
                      asChild
                      className={navigationMenuTriggerStyle()}
                    >
                      <Link to="/api-keys">API Keys</Link>
                    </NavigationMenuLink>
                  </NavigationMenuItem>
                </>
              )}
            </NavigationMenuList>
          </NavigationMenu>
        </div>

        {/* Auth Buttons */}
        <div className="flex items-center gap-4">
          {user ? (
            <>
              {/* User Avatar and Menu */}
              <div className="flex items-center gap-3">
                <div className="hidden md:flex flex-col items-end">
                  <span className="text-sm font-medium">
                    {user.name || user.nickname || user.email}
                  </span>
                  {user.email && user.name && (
                    <span className="text-xs text-muted-foreground">
                      {user.email}
                    </span>
                  )}
                </div>
                <Avatar>
                  <AvatarImage src={user.picture} alt={user.name || "User"} />
                  <AvatarFallback>
                    {getInitials(user.name || user.email)}
                  </AvatarFallback>
                </Avatar>
              </div>

              {/* Logout Button */}
              <Link to="/auth/logout">
                <Button variant="destructive">Logout</Button>
              </Link>
            </>
          ) : (
            <>
              <Link to="/pricing" className="md:hidden">
                <Button variant="ghost">Pricing</Button>
              </Link>
              {/* Login and Sign Up Buttons */}
              <Link to="/auth/login">
                <Button variant="ghost">Login</Button>
              </Link>
              <Link to="/auth/signup">
                <Button>Sign Up</Button>
              </Link>
            </>
          )}
        </div>
      </div>
    </header>
  );
}
