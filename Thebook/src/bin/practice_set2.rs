//create a CLI BANK
struct AccountDetails {
    status : bool,
    username : String,
    account_no : String ,
    pin : u16,
    balance : u64,
}


impl AccountDetails {
        fn check_pin(&self,entered_pin:u16) -> bool {
             self.pin == entered_pin 
        }
        
        fn _new_account(username:String,account_no:String ,) -> Self {
          
          Self {
            status: true ,
            username,
            account_no,
            pin: 0000 ,
            balance : 0000,
            
          }
        }

        fn _detail_acc(&self) -> (bool,&str,&str) {
               (self.status,&self.username,&self.account_no)
        }

        fn bal(&self) -> u64 {
          self.balance
        }

        fn disp_bal(&self) -> (u64,u64) {
              let rup = self.balance/100;
              let paise = self.balance%100 ;
              (rup,paise)
        }
    }


fn main() {
      let  entered_pin  = 2234; 
      let user1 = AccountDetails {
        status : true,
        username : String::from("jassi"),
        account_no: String::from("9876543456789"),
        pin : 2234,
        balance : 10000,
      };
      if user1.check_pin(entered_pin){
        println!("ok you have the acces")
      } else { 
        println!("accessdenied");
      }
      
      let (rup,paise) = user1.disp_bal();
      println!("your account balance is ₹{}.{}",rup,paise);
}